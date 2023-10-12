/// 0 2 5 9
///
/// 1 4 8
///
/// 3 7
///
/// 6
///
/// row and column start at 1, returned flat index starts at 0.
pub fn triangular_matrix_row_column_index_to_flat_index(row: usize, column: usize) -> usize {
    let column = column;
    let row = row - 1;

    let column = column + row;

    let triangular_number = column * (column + 1) / 2;
    let index = triangular_number - row - 1; // - 1 for zero indexing
    index
}

/// Modified version of https://stackoverflow.com/a/9674523
///
/// Returns (row, column) where row and column start at 1.
pub fn traingular_matrix_flat_index_to_row_column_index(index: usize) -> (usize, usize) {
    let row = (-0.5 + ((0.25_f32 + 2_f32 * index as f32) as f32).sqrt()) as i32;
    let triangular_number = row * (row + 1) / 2;
    let column = index as i32 - triangular_number;
    (1 + (row - column) as usize, 1 + column as usize) // + 1 for one indexing to conform to puzzle example
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangular_matrix_row_column_index_to_flat_index_works() {
        let mut indices = Vec::new();
        for row in 1..=6 {
            indices.push(triangular_matrix_row_column_index_to_flat_index(row, 1));
        }
        assert_eq!(indices, vec![0, 1, 3, 6, 10, 15]);

        let mut indices = Vec::new();
        for column in 1..=6 {
            indices.push(triangular_matrix_row_column_index_to_flat_index(1, column));
        }
        assert_eq!(indices, vec![0, 2, 5, 9, 14, 20]);

        assert_eq!(
            triangular_matrix_row_column_index_to_flat_index(2, 2),
            4,
            "row 2 column 2"
        );
        assert_eq!(
            triangular_matrix_row_column_index_to_flat_index(3, 3),
            12,
            "row 3 column 3"
        );
        assert_eq!(
            triangular_matrix_row_column_index_to_flat_index(4, 4),
            24,
            "row 4 column 4"
        );
        assert_eq!(
            triangular_matrix_row_column_index_to_flat_index(5, 5),
            40,
            "row 5 column 5"
        );
        assert_eq!(
            triangular_matrix_row_column_index_to_flat_index(6, 6),
            60,
            "row 6 column 6"
        );
        assert_eq!(
            triangular_matrix_row_column_index_to_flat_index(9, 3),
            57,
            "row 9 column 3"
        );
    }

    #[test]
    fn test_back_and_forth_conversion() {
        for i in 0..10 {
            let (row, column) = traingular_matrix_flat_index_to_row_column_index(i);
            assert_eq!(
                triangular_matrix_row_column_index_to_flat_index(row, column),
                i
            );
        }
    }
}
