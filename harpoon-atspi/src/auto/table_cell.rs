// Generated by gir (https://github.com/gtk-rs/gir @ f64f90a)
// from girs (@ 4b6cabc)
// DO NOT EDIT

use crate::Accessible;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::mem;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "AtspiTableCell")]
    pub struct TableCell(Interface<ffi::AtspiTableCell>);

    match fn {
        type_ => || ffi::atspi_table_cell_get_type(),
    }
}

pub const NONE_TABLE_CELL: Option<&TableCell> = None;

pub trait TableCellExt: 'static {
    #[doc(alias = "atspi_table_cell_get_column_header_cells")]
    #[doc(alias = "get_column_header_cells")]
    fn column_header_cells(&self) -> Result<Vec<Accessible>, glib::Error>;

    #[doc(alias = "atspi_table_cell_get_column_index")]
    #[doc(alias = "get_column_index")]
    fn column_index(&self) -> Result<i32, glib::Error>;

    #[doc(alias = "atspi_table_cell_get_column_span")]
    #[doc(alias = "get_column_span")]
    fn column_span(&self) -> Result<i32, glib::Error>;

    #[doc(alias = "atspi_table_cell_get_position")]
    #[doc(alias = "get_position")]
    fn position(&self) -> Result<(i32, i32, i32), glib::Error>;

    #[doc(alias = "atspi_table_cell_get_row_column_span")]
    #[doc(alias = "get_row_column_span")]
    fn row_column_span(&self) -> Result<(i32, i32, i32, i32), glib::Error>;

    #[doc(alias = "atspi_table_cell_get_row_header_cells")]
    #[doc(alias = "get_row_header_cells")]
    fn row_header_cells(&self) -> Result<Vec<Accessible>, glib::Error>;

    #[doc(alias = "atspi_table_cell_get_row_span")]
    #[doc(alias = "get_row_span")]
    fn row_span(&self) -> Result<i32, glib::Error>;

    #[doc(alias = "atspi_table_cell_get_table")]
    #[doc(alias = "get_table")]
    fn table(&self) -> Result<Accessible, glib::Error>;
}

impl<O: IsA<TableCell>> TableCellExt for O {
    fn column_header_cells(&self) -> Result<Vec<Accessible>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_table_cell_get_column_header_cells(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(FromGlibPtrContainer::from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn column_index(&self) -> Result<i32, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_table_cell_get_column_index(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn column_span(&self) -> Result<i32, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_table_cell_get_column_span(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn position(&self) -> Result<(i32, i32, i32), glib::Error> {
        unsafe {
            let mut row = mem::MaybeUninit::uninit();
            let mut column = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_table_cell_get_position(self.as_ref().to_glib_none().0, row.as_mut_ptr(), column.as_mut_ptr(), &mut error);
            let row = row.assume_init();
            let column = column.assume_init();
            if error.is_null() { Ok((ret, row, column)) } else { Err(from_glib_full(error)) }
        }
    }

    fn row_column_span(&self) -> Result<(i32, i32, i32, i32), glib::Error> {
        unsafe {
            let mut row = mem::MaybeUninit::uninit();
            let mut column = mem::MaybeUninit::uninit();
            let mut row_span = mem::MaybeUninit::uninit();
            let mut column_span = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let _ = ffi::atspi_table_cell_get_row_column_span(self.as_ref().to_glib_none().0, row.as_mut_ptr(), column.as_mut_ptr(), row_span.as_mut_ptr(), column_span.as_mut_ptr(), &mut error);
            let row = row.assume_init();
            let column = column.assume_init();
            let row_span = row_span.assume_init();
            let column_span = column_span.assume_init();
            if error.is_null() { Ok((row, column, row_span, column_span)) } else { Err(from_glib_full(error)) }
        }
    }

    fn row_header_cells(&self) -> Result<Vec<Accessible>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_table_cell_get_row_header_cells(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(FromGlibPtrContainer::from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn row_span(&self) -> Result<i32, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_table_cell_get_row_span(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn table(&self) -> Result<Accessible, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::atspi_table_cell_get_table(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}

impl fmt::Display for TableCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TableCell")
    }
}
