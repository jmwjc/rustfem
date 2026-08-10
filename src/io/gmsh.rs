use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;

use crate::element::Element;

// ============================================================================
// Public API
// ============================================================================

/// Parse a GMSH v4 `.msh` file and return a map `physical-group-id → Vec<T>`.
///
/// The element type `T` must implement [`Element<D, P, N>`] which includes
/// `const GMSH_ELEMENT_TYPE: i32` and a
/// `fn from_gmsh(&[usize], &HashMap<usize, Node<D>>) -> Option<Self>`
/// constructor.
///
/// # Memory
///
/// Only the node coordinate table and the resulting element vectors are held
/// in memory; the file is consumed via a buffered line reader and element
/// vectors are pre-allocated using the element count declared in the file
/// header.
///
/// # Panics
///
/// Panics if the file is not GMSH v4, is malformed, or if a required section
/// is missing.
pub fn read_gmsh<T, const D: usize, const P: usize, const N: usize>(
    filepath: impl AsRef<Path>,
) -> HashMap<i32, Vec<T>>
where
    T: Element<D, P, N>,
{
    let file = File::open(filepath.as_ref()).expect("cannot open .msh file");
    let file_len = file.metadata().map(|m| m.len()).unwrap_or(0);
    let mut reader = BufReader::with_capacity(64 * 1024, file); // 64 KiB buffer

    // ---------- $MeshFormat ----------
    skip_to_section(&mut reader, "$MeshFormat");
    let (version, _binary, _data_size) = read_mesh_format(&mut reader);
    assert!(
        (4.0..5.0).contains(&version),
        "only GMSH v4 format is supported (got {version})"
    );

    // ---------- $PhysicalNames (optional) ----------
    let _physical_names: HashMap<i32, String> =
        if peek_section(&mut reader).as_deref() == Some("$PhysicalNames") {
            skip_to_section(&mut reader, "$PhysicalNames");
            read_physical_names(&mut reader)
        } else {
            HashMap::new()
        };

    // ---------- $Entities ----------
    let entity_to_phys: HashMap<(i32, i32), i32> =
        if peek_section(&mut reader).as_deref() == Some("$Entities") {
            skip_to_section(&mut reader, "$Entities");
            read_entities(&mut reader)
        } else {
            panic!("$Entities section not found – only GMSH v4 is supported");
        };

    // ---------- $Nodes ----------
    skip_to_section(&mut reader, "$Nodes");
    let nodes = read_nodes_v4::<D>(&mut reader);

    // ---------- $Elements ----------
    skip_to_section(&mut reader, "$Elements");
    let elements = read_elements_v4::<T, D, P, N>(&mut reader, &nodes, &entity_to_phys, file_len);

    elements
}

// ============================================================================
// Internal helpers – section navigation
// ============================================================================

/// Advance the reader past the line that contains `$SectionName` (inclusive).
fn skip_to_section(reader: &mut BufReader<File>, section: &str) {
    let mut line = String::new();
    loop {
        line.clear();
        let n = reader
            .read_line(&mut line)
            .expect("I/O error reading .msh");
        assert_ne!(n, 0, "unexpected EOF while looking for {section}");
        if line.trim() == section {
            return;
        }
    }
}

/// Peek at the next non-empty section tag without consuming it.
fn peek_section(reader: &mut BufReader<File>) -> Option<String> {
    let pos = reader.stream_position().ok()?;
    let mut line = String::new();
    loop {
        line.clear();
        let n = reader.read_line(&mut line).ok()?;
        if n == 0 {
            break;
        }
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        reader.seek(SeekFrom::Start(pos)).ok()?;
        return Some(trimmed.to_string());
    }
    reader.seek(SeekFrom::Start(pos)).ok()?;
    None
}

// ============================================================================
// $MeshFormat
// ============================================================================

fn read_mesh_format(reader: &mut BufReader<File>) -> (f64, bool, usize) {
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("I/O error reading $MeshFormat");
    let parts: Vec<&str> = line.split_whitespace().collect();
    assert!(parts.len() >= 3, "malformed $MeshFormat line");
    let version: f64 = parts[0].parse().expect("invalid mesh format version");
    let binary: bool = parts[1] != "0";
    let _data_size: usize = parts[2].parse().unwrap_or(8);
    assert!(!binary, "binary GMSH files are not supported");
    let mut end_line = String::new();
    reader.read_line(&mut end_line).expect("I/O error");
    assert!(
        end_line.trim() == "$EndMeshFormat",
        "expected $EndMeshFormat"
    );
    (version, binary, _data_size)
}

// ============================================================================
// $PhysicalNames
// ============================================================================

fn read_physical_names(reader: &mut BufReader<File>) -> HashMap<i32, String> {
    let mut line = String::new();
    reader.read_line(&mut line).expect("I/O error");
    let count: usize = line
        .trim()
        .parse()
        .expect("invalid $PhysicalNames count");
    let mut map = HashMap::with_capacity(count);
    for _ in 0..count {
        line.clear();
        reader.read_line(&mut line).expect("I/O error");
        let parts: Vec<&str> = line.split_whitespace().collect();
        assert!(parts.len() >= 3, "malformed $PhysicalNames entry");
        let phys_tag: i32 = parts[1].parse().expect("invalid physical tag");
        // Re-join everything after the first two tokens to get the name
        let name_start = parts[0].len() + parts[1].len() + 2;
        let name = line[name_start..].trim().trim_matches('"').to_string();
        map.insert(phys_tag, name);
    }
    let mut end_line = String::new();
    reader.read_line(&mut end_line).expect("I/O error");
    assert!(
        end_line.trim() == "$EndPhysicalNames",
        "expected $EndPhysicalNames"
    );
    map
}

// ============================================================================
// $Entities
// ============================================================================

/// Build `(dim, entity_tag) → physical_group` mapping.
fn read_entities(reader: &mut BufReader<File>) -> HashMap<(i32, i32), i32> {
    let mut line = String::new();
    reader.read_line(&mut line).expect("I/O error");
    let counts: Vec<usize> = line
        .split_whitespace()
        .map(|s| s.parse().expect("invalid $Entities counts"))
        .collect();
    assert_eq!(counts.len(), 4, "expected 4 entity counts");

    let total = counts.iter().sum::<usize>();
    let mut map = HashMap::with_capacity(total);

    read_entity_block(reader, &mut map, 0, counts[0], 3);
    read_entity_block(reader, &mut map, 1, counts[1], 6);
    read_entity_block(reader, &mut map, 2, counts[2], 6);
    read_entity_block(reader, &mut map, 3, counts[3], 6);

    let mut end_line = String::new();
    reader.read_line(&mut end_line).expect("I/O error");
    assert!(end_line.trim() == "$EndEntities", "expected $EndEntities");

    map
}

fn read_entity_block(
    reader: &mut BufReader<File>,
    map: &mut HashMap<(i32, i32), i32>,
    dim: i32,
    count: usize,
    coord_fields: usize,
) {
    let mut line = String::new();
    for _ in 0..count {
        line.clear();
        reader.read_line(&mut line).expect("I/O error");
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let tag: i32 = tokens[0].parse().expect("invalid entity tag");
        let idx = 1 + coord_fields;
        let num_phys: usize = tokens[idx].parse().expect("invalid numPhysicalTags");
        let phys_tag: i32 = if num_phys > 0 {
            tokens[idx + 1].parse().expect("invalid physicalTag")
        } else {
            0
        };
        map.insert((dim, tag), phys_tag);
    }
}

// ============================================================================
// $Nodes (v4)
// ============================================================================

fn read_nodes_v4<const D: usize>(
    reader: &mut BufReader<File>,
) -> HashMap<usize, crate::node::Node<D>> {
    let mut line = String::new();
    reader.read_line(&mut line).expect("I/O error");
    let header: Vec<usize> = line
        .split_whitespace()
        .map(|s| s.parse().expect("invalid $Nodes header"))
        .collect();
    assert_eq!(header.len(), 4, "expected 4 $Nodes header fields");
    let num_entity_blocks = header[0];
    let num_nodes = header[1];

    let mut nodes = HashMap::with_capacity(num_nodes);

    for _ in 0..num_entity_blocks {
        line.clear();
        reader.read_line(&mut line).expect("I/O error");
        let block_header: Vec<&str> = line.split_whitespace().collect();
        assert!(
            block_header.len() >= 4,
            "malformed node entity block header"
        );
        let _entity_dim: i32 = block_header[0].parse().expect("invalid entityDim");
        let parametric: i32 = block_header[2].parse().expect("invalid parametric flag");
        let num_nodes_in_block: usize =
            block_header[3].parse().expect("invalid node count");

        // Auto-detect: read the first data line to determine format.
        line.clear();
        reader.read_line(&mut line).expect("I/O error");
        let first_tokens: Vec<&str> = line.split_whitespace().collect();

        // If first line has >1 tokens, it's compact format: "tag X Y Z" per line.
        // Otherwise (1 token) it's separated: tags first, then coordinates.
        let separated = first_tokens.len() == 1;

        assert_eq!(
            parametric,
            0,
            "parametric nodes only supported in separated format"
        );

        if separated {
            // Separated format: tags then coordinates.
            // Re-process the first line which contains the first tag(s).
            let mut tags: Vec<usize> = Vec::with_capacity(num_nodes_in_block);
            let first_tags: Vec<&str> = first_tokens;
            for t in &first_tags {
                if tags.len() >= num_nodes_in_block {
                    break;
                }
                if !t.is_empty() {
                    tags.push(t.parse().expect("invalid node tag"));
                }
            }
            while tags.len() < num_nodes_in_block {
                line.clear();
                reader.read_line(&mut line).expect("I/O error");
                let toks: Vec<&str> = line.split_whitespace().collect();
                for t in &toks {
                    if tags.len() >= num_nodes_in_block {
                        break;
                    }
                    if !t.is_empty() {
                        tags.push(t.parse().expect("invalid node tag"));
                    }
                }
            }
            // Now read coordinates.
            for i in 0..num_nodes_in_block {
                line.clear();
                reader.read_line(&mut line).expect("I/O error");
                let toks: Vec<&str> = line.split_whitespace().collect();
                let ncoords = D.min(toks.len());
                let mut coords = [0.0_f64; D];
                for d in 0..ncoords {
                    coords[d] = toks[d].parse().expect("invalid node coordinate");
                }
                nodes.insert(tags[i], crate::node::Node::new(tags[i], coords));
            }
        } else {
            // Compact format: each line is "tag x y z".
            let first_id: usize = first_tokens[0].parse().expect("invalid node tag");
            let ncoords = D.min(first_tokens.len() - 1);
            let mut first_coords = [0.0_f64; D];
            for d in 0..ncoords {
                first_coords[d] = first_tokens[1 + d]
                    .parse()
                    .expect("invalid node coordinate");
            }
            nodes.insert(first_id, crate::node::Node::new(first_id, first_coords));

            for _node_idx in 1..num_nodes_in_block {
                line.clear();
                reader.read_line(&mut line).expect("I/O error");
                let toks: Vec<&str> = line.split_whitespace().collect();
                let id: usize = toks[0].parse().expect("invalid node tag");
                let n = D.min(toks.len() - 1);
                let mut coords = [0.0_f64; D];
                for d in 0..n {
                    coords[d] = toks[1 + d]
                        .parse()
                        .expect("invalid node coordinate");
                }
                nodes.insert(id, crate::node::Node::new(id, coords));
            }
        }
    }

    let mut end_line = String::new();
    reader.read_line(&mut end_line).expect("I/O error");
    assert!(end_line.trim() == "$EndNodes", "expected $EndNodes");

    nodes
}

// ============================================================================
// $Elements (v4)
// ============================================================================

fn read_elements_v4<T, const D: usize, const P: usize, const N: usize>(
    reader: &mut BufReader<File>,
    nodes: &HashMap<usize, crate::node::Node<D>>,
    entity_to_phys: &HashMap<(i32, i32), i32>,
    file_len: u64,
) -> HashMap<i32, Vec<T>>
where
    T: Element<D, P, N>,
{
    let mut line = String::new();
    reader.read_line(&mut line).expect("I/O error");
    let header: Vec<usize> = line
        .split_whitespace()
        .map(|s| s.parse().expect("invalid $Elements header"))
        .collect();
    assert_eq!(header.len(), 4, "expected 4 $Elements header fields");
    let num_entity_blocks = header[0];

    let target_type = T::GMSH_ELEMENT_TYPE;
    let nodes_per_elem = N;

    let est_elem_count = (file_len / 40) as usize;
    let mut phys_to_elems: HashMap<i32, Vec<T>> = HashMap::new();

    for _block in 0..num_entity_blocks {
        line.clear();
        reader.read_line(&mut line).expect("I/O error");
        let block_header: Vec<&str> = line.split_whitespace().collect();
        assert!(
            block_header.len() >= 4,
            "malformed element entity block header"
        );
        let entity_dim: i32 = block_header[0].parse().expect("invalid entityDim");
        let entity_tag: i32 = block_header[1].parse().expect("invalid entityTag");
        let elem_type: i32 = block_header[2].parse().expect("invalid elementType");
        let num_elems_in_block: usize = block_header[3].parse().expect("invalid element count");

        let phys_group = entity_to_phys
            .get(&(entity_dim, entity_tag))
            .copied()
            .unwrap_or(0);

        if elem_type == target_type {
            let phys_len = phys_to_elems.len().max(1);
            let bucket = phys_to_elems
                .entry(phys_group)
                .or_insert_with(|| Vec::with_capacity(est_elem_count / phys_len));
            bucket.reserve(num_elems_in_block);

            for _elem in 0..num_elems_in_block {
                line.clear();
                reader.read_line(&mut line).expect("I/O error");
                let tokens: Vec<&str> = line.split_whitespace().collect();
                let node_ids: Vec<usize> = tokens[1..]
                    .iter()
                    .take(nodes_per_elem)
                    .map(|s| s.parse().expect("invalid node tag"))
                    .collect();
                if let Some(elm) = T::from_gmsh(&node_ids, nodes) {
                    bucket.push(elm);
                }
            }
        } else {
            // Skip this block – not the element type we're collecting.
            for _elem in 0..num_elems_in_block {
                line.clear();
                reader.read_line(&mut line).expect("I/O error");
            }
        }
    }

    let mut end_line = String::new();
    reader.read_line(&mut end_line).expect("I/O error");
    assert!(end_line.trim() == "$EndElements", "expected $EndElements");

    for v in phys_to_elems.values_mut() {
        v.shrink_to_fit();
    }

    phys_to_elems
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::seg2::Seg2;
    use std::io::Write;

    fn minimal_msh_v4() -> String {
        concat!(
            "$MeshFormat\n",
            "4.1 0 8\n",
            "$EndMeshFormat\n",
            "$PhysicalNames\n",
            "1\n",
            "1 1 \"truss\"\n",
            "$EndPhysicalNames\n",
            "$Entities\n",
            "0 1 0 0\n",
            "1 0.0 0.0 0.0 1.0 0.0 0.0 1 1 2 1 2\n",
            "$EndEntities\n",
            "$Nodes\n",
            "2 4 1 4\n",
            "1 1 0 2\n",
            "1 0.0 0.0 0.0\n",
            "2 1.0 0.0 0.0\n",
            "1 1 0 2\n",
            "3 0.0 0.0 0.0\n",
            "4 1.0 0.0 0.0\n",
            "$EndNodes\n",
            "$Elements\n",
            "2 2 1 2\n",
            "1 1 1 1\n",
            "1 1 2\n",
            "1 1 1 1\n",
            "2 3 4\n",
            "$EndElements\n",
        )
        .to_string()
    }

    #[test]
    fn test_parse_minimal_2d_truss() {
        let mut tmp = tempfile::Builder::new()
            .suffix(".msh")
            .tempfile()
            .unwrap();
        write!(tmp, "{}", minimal_msh_v4()).unwrap();
        let path = tmp.path().to_path_buf();

        let result: HashMap<i32, Vec<Seg2<2>>> = read_gmsh(&path);

        assert!(result.contains_key(&1));
        let total: usize = result.values().map(|v| v.len()).sum();
        assert_eq!(total, 2);
    }

    /// Generate a 1D `.msh` file with 3 nodes and 2 line elements (Seg2<1>).
    fn make_1d_msh() -> String {
        concat!(
            "$MeshFormat\n",
            "4.1 0 8\n",
            "$EndMeshFormat\n",
            "$PhysicalNames\n",
            "1\n",
            "1 1 \"truss_1d\"\n",
            "$EndPhysicalNames\n",
            "$Entities\n",
            "0 1 0 0\n",
            // curve entity: tag=1, bbox=(0,0,0)-(1,0,0), physTag=1,
            // numBoundingPoints=2 {1,2}
            "1 0.0 0.0 0.0 1.0 0.0 0.0 1 1 2 1 2\n",
            "$EndEntities\n",
            "$Nodes\n",
            "1 3 1 3\n",
            // entity block: entityDim=1, entityTag=1, parametric=0, numNodes=3
            "1 1 0 3\n",
            "1 0.0 0.0 0.0\n",
            "2 0.5 0.0 0.0\n",
            "3 1.0 0.0 0.0\n",
            "$EndNodes\n",
            "$Elements\n",
            "1 2 1 2\n",
            // element block: entityDim=1, entityTag=1, elemType=1 (2-node line),
            // numElem=2
            "1 1 1 2\n",
            "1 1 2\n",
            "2 2 3\n",
            "$EndElements\n",
        )
        .to_string()
    }

    #[test]
    fn test_parse_1d_truss() {
        let mut tmp = tempfile::Builder::new()
            .suffix(".msh")
            .tempfile()
            .unwrap();
        write!(tmp, "{}", make_1d_msh()).unwrap();
        let path = tmp.path().to_path_buf();

        let result: HashMap<i32, Vec<Seg2<1>>> = read_gmsh(&path);

        // physical group 1 should contain 2 elements
        assert!(result.contains_key(&1));
        let elems = &result[&1];
        assert_eq!(elems.len(), 2);

        // Check element connectivity:
        // elem 1: nodes 1-2 -> coords [0.0], [0.5]
        assert_eq!(elems[0].id(), [1, 2]);
        // elem 2: nodes 2-3 -> coords [0.5], [1.0]
        assert_eq!(elems[1].id(), [2, 3]);

        // Check jacobe: half-length of each element
        // elem 1: length=0.5, jacobe=0.25
        assert!((elems[0].jacobe([0.0]) - 0.25).abs() < 1e-12);
        // elem 2: length=0.5, jacobe=0.25
        assert!((elems[1].jacobe([0.0]) - 0.25).abs() < 1e-12);
    }

    /// Read boundary `Seg2<2>` line elements from the real `test/msh/patchtest.msh`
    /// file.  The file contains a 1×1 square meshed with quads/triangles in the
    /// interior (physical group 2 = "Ω") and 40 2-node line elements on the
    /// boundary (physical group 1 = "Γᵍ").
    #[test]
    fn test_parse_patchtest_boundary_seg2() {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let path = std::path::PathBuf::from(manifest_dir)
            .join("test")
            .join("msh")
            .join("patchtest.msh");

        let result: HashMap<i32, Vec<Seg2<2>>> = read_gmsh(&path);

        // Boundary edges are in physical group 1.
        assert!(result.contains_key(&1));
        assert!(!result.contains_key(&2)); // group 2 has triangles, not Seg2

        let edges = &result[&1];
        // 4 curves × 10 segments each = 40 2-node line elements.
        assert_eq!(edges.len(), 40);

        // Each element should reference valid node ids (1..165).
        for elm in edges {
            let [n1, n2] = elm.id();
            assert!(n1 >= 1 && n1 <= 165, "node {n1} out of range");
            assert!(n2 >= 1 && n2 <= 165, "node {n2} out of range");
            assert_ne!(n1, n2);
            // jacobe (half-length) must be positive.
            assert!(elm.jacobe([0.0]) > 0.0);
        }
    }
}
