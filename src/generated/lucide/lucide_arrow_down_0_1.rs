use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m3 16 4 4 4-4" ></ path > < path d = "M7 20V4" ></ path > < rect x = "15" y = "4" height = "6" width = "4" ry = "2" ></ rect > < path d = "M17 20v-6h-2" ></ path > < path d = "M15 20h4" ></ path > < / > } } pub const LUCIDE_ARROW_DOWN_0_1 : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;