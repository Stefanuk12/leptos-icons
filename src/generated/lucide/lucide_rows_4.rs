use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" x = "3" rx = "2" y = "3" height = "18" ></ rect > < path d = "M21 7.5H3" ></ path > < path d = "M21 12H3" ></ path > < path d = "M21 16.5H3" ></ path > < / > } } pub const LUCIDE_ROWS_4 : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;