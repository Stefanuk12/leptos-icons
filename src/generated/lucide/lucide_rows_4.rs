use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" x = "3" height = "18" width = "18" y = "3" ></ rect > < path d = "M21 7.5H3" ></ path > < path d = "M21 12H3" ></ path > < path d = "M21 16.5H3" ></ path > < / > } } pub const LUCIDE_ROWS_4 : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;