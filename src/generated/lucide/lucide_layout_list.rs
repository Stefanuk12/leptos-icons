use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "7" x = "3" height = "7" rx = "1" y = "3" ></ rect > < rect height = "7" y = "14" width = "7" rx = "1" x = "3" ></ rect > < path d = "M14 4h7" ></ path > < path d = "M14 9h7" ></ path > < path d = "M14 15h7" ></ path > < path d = "M14 20h7" ></ path > < / > } } pub const LUCIDE_LAYOUT_LIST : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24")] } ;