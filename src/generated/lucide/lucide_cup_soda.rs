use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m6 8 1.75 12.28a2 2 0 0 0 2 1.72h4.54a2 2 0 0 0 2-1.72L18 8" ></ path > < path d = "M5 8h14" ></ path > < path d = "M7 15a6.47 6.47 0 0 1 5 0 6.47 6.47 0 0 0 5 0" ></ path > < path d = "m12 8 1-6h2" ></ path > < / > } } pub const LUCIDE_CUP_SODA : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linecap" , "round")] } ;