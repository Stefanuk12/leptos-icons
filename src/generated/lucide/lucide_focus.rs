use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "3" cy = "12" ></ circle > < path d = "M3 7V5a2 2 0 0 1 2-2h2" ></ path > < path d = "M17 3h2a2 2 0 0 1 2 2v2" ></ path > < path d = "M21 17v2a2 2 0 0 1-2 2h-2" ></ path > < path d = "M7 21H5a2 2 0 0 1-2-2v-2" ></ path > < / > } } pub const LUCIDE_FOCUS : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;