use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" ></ path > < path d = "M3 3v5h5" ></ path > < path d = "M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16" ></ path > < path d = "M16 16h5v5" ></ path > < / > } } pub const LUCIDE_REFRESH_CCW : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;