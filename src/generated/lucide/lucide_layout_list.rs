use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" rx = "1" height = "7" width = "7" y = "3" ></ rect > < rect x = "3" width = "7" height = "7" rx = "1" y = "14" ></ rect > < path d = "M14 4h7" ></ path > < path d = "M14 9h7" ></ path > < path d = "M14 15h7" ></ path > < path d = "M14 20h7" ></ path > < / > } } pub const LUCIDE_LAYOUT_LIST : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;