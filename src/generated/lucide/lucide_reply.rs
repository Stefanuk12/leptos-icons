use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "9 17 4 12 9 7" ></ polyline > < path d = "M20 18v-2a4 4 0 0 0-4-4H4" ></ path > < / > } } pub const LUCIDE_REPLY : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2")] } ;