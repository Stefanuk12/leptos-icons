use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m3 8 4-4 4 4" ></ path > < path d = "M7 4v16" ></ path > < path d = "M17 10V4h-2" ></ path > < path d = "M15 10h4" ></ path > < rect height = "6" width = "4" x = "15" y = "14" ry = "2" ></ rect > < / > } } pub const LUCIDE_ARROW_UP_1_0 : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;