use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" rx = "2" height = "18" x = "3" y = "3" ></ rect > < path d = "M21 7.5H3" ></ path > < path d = "M21 12H3" ></ path > < path d = "M21 16.5H3" ></ path > < / > } } pub const LUCIDE_ROWS_4 : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke" , "currentColor")] } ;