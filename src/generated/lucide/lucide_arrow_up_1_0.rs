use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m3 8 4-4 4 4" ></ path > < path d = "M7 4v16" ></ path > < path d = "M17 10V4h-2" ></ path > < path d = "M15 10h4" ></ path > < rect width = "4" y = "14" x = "15" height = "6" ry = "2" ></ rect > < / > } } pub const LUCIDE_ARROW_UP_1_0 : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;