use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M7 9h.01" ></ path > < path d = "M16.75 12H22l-3.5 7-3.09-4.32" ></ path > < path d = "M18 9.5l-4 8-10.39-5.2a2.92 2.92 0 0 1-1.3-3.91L3.69 5.6a2.92 2.92 0 0 1 3.92-1.3Z" ></ path > < path d = "M2 19h3.76a2 2 0 0 0 1.8-1.1L9 15" ></ path > < path d = "M2 21v-4" ></ path > < / > } } pub const LUCIDE_CCTV : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;