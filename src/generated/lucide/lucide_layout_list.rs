use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "7" x = "3" y = "3" rx = "1" width = "7" ></ rect > < rect y = "14" rx = "1" x = "3" height = "7" width = "7" ></ rect > < path d = "M14 4h7" ></ path > < path d = "M14 9h7" ></ path > < path d = "M14 15h7" ></ path > < path d = "M14 20h7" ></ path > < / > } } pub const LUCIDE_LAYOUT_LIST : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("height" , "24")] } ;