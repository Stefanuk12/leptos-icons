use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" rx = "2" y = "3" x = "3" height = "18" ></ rect > < path d = "M21 9H3" ></ path > < path d = "M21 15H3" ></ path > < / > } } pub const LUCIDE_ROWS_3 : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;