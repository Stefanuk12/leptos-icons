use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "15 14 20 9 15 4" ></ polyline > < path d = "M4 20v-7a4 4 0 0 1 4-4h12" ></ path > < / > } } pub const LucideCornerUpRight : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;