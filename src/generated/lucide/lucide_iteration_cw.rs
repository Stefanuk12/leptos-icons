use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 10c0-4.4 3.6-8 8-8s8 3.6 8 8-3.6 8-8 8H4" ></ path > < polyline points = "8 22 4 18 8 14" ></ polyline > < / > } } pub const LUCIDE_ITERATION_CW : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round")] } ;