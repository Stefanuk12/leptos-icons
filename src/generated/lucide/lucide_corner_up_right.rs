use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "15 14 20 9 15 4" ></ polyline > < path d = "M4 20v-7a4 4 0 0 1 4-4h12" ></ path > < / > } } pub const LUCIDE_CORNER_UP_RIGHT : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke" , "currentColor") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none")] } ;