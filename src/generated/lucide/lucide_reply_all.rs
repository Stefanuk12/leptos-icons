use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "7 17 2 12 7 7" ></ polyline > < polyline points = "12 17 7 12 12 7" ></ polyline > < path d = "M22 18v-2a4 4 0 0 0-4-4H7" ></ path > < / > } } pub const LUCIDE_REPLY_ALL : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor")] } ;