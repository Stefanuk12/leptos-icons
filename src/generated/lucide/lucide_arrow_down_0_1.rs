use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m3 16 4 4 4-4" ></ path > < path d = "M7 20V4" ></ path > < rect y = "4" ry = "2" width = "4" height = "6" x = "15" ></ rect > < path d = "M17 20v-6h-2" ></ path > < path d = "M15 20h4" ></ path > < / > } } pub const LUCIDE_ARROW_DOWN_0_1 : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;