use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "7" x = "3" height = "7" y = "3" rx = "1" ></ rect > < rect height = "7" x = "3" y = "14" rx = "1" width = "7" ></ rect > < path d = "M14 4h7" ></ path > < path d = "M14 9h7" ></ path > < path d = "M14 15h7" ></ path > < path d = "M14 20h7" ></ path > < / > } } pub const LUCIDE_LAYOUT_LIST : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("stroke-width" , "2")] } ;