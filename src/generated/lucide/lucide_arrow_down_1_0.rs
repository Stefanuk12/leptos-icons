use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m3 16 4 4 4-4" ></ path > < path d = "M7 20V4" ></ path > < path d = "M17 10V4h-2" ></ path > < path d = "M15 10h4" ></ path > < rect y = "14" x = "15" ry = "2" height = "6" width = "4" ></ rect > < / > } } pub const LUCIDE_ARROW_DOWN_1_0 : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("height" , "24") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;