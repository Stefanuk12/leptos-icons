use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m3 8 4-4 4 4" ></ path > < path d = "M7 4v16" ></ path > < rect height = "6" x = "15" y = "4" width = "4" ry = "2" ></ rect > < path d = "M17 20v-6h-2" ></ path > < path d = "M15 20h4" ></ path > < / > } } pub const LUCIDE_ARROW_UP_0_1 : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;