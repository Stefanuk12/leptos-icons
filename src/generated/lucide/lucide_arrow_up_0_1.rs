use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m3 8 4-4 4 4" ></ path > < path d = "M7 4v16" ></ path > < rect y = "4" height = "6" ry = "2" x = "15" width = "4" ></ rect > < path d = "M17 20v-6h-2" ></ path > < path d = "M15 20h4" ></ path > < / > } } pub const LUCIDE_ARROW_UP_0_1 : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24")] } ;