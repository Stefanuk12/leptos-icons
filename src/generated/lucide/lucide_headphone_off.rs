use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 14h-1.343" ></ path > < path d = "M9.128 3.47A9 9 0 0 1 21 12v3.343" ></ path > < path d = "m2 2 20 20" ></ path > < path d = "M20.414 20.414A2 2 0 0 1 19 21h-1a2 2 0 0 1-2-2v-3" ></ path > < path d = "M3 14h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-7a9 9 0 0 1 2.636-6.364" ></ path > < / > } } pub const LUCIDE_HEADPHONE_OFF : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;