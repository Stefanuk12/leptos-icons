use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "7" rx = "1" width = "7" x = "3" y = "3" ></ rect > < rect height = "7" y = "14" width = "7" x = "3" rx = "1" ></ rect > < path d = "M14 4h7" ></ path > < path d = "M14 9h7" ></ path > < path d = "M14 15h7" ></ path > < path d = "M14 20h7" ></ path > < / > } } pub const LUCIDE_LAYOUT_LIST : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor")] } ;