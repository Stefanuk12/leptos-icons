use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10.083 9A6.002 6.002 0 0 1 16 4a4.243 4.243 0 0 0 6 6c0 2.22-1.206 4.16-3 5.197" ></ path > < path d = "M3 20a5 5 0 1 1 8.9-4H13a3 3 0 0 1 2 5.24" ></ path > < path d = "M11 20v2" ></ path > < path d = "M7 19v2" ></ path > < / > } } pub const LUCIDE_CLOUD_MOON_RAIN : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;