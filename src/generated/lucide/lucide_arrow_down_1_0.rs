use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m3 16 4 4 4-4" ></ path > < path d = "M7 20V4" ></ path > < path d = "M17 10V4h-2" ></ path > < path d = "M15 10h4" ></ path > < rect y = "14" width = "4" x = "15" height = "6" ry = "2" ></ rect > < / > } } pub const LUCIDE_ARROW_DOWN_1_0 : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;