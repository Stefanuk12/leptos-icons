use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6.3 20.3a2.4 2.4 0 0 0 3.4 0L12 18l-6-6-2.3 2.3a2.4 2.4 0 0 0 0 3.4Z" ></ path > < path d = "m2 22 3-3" ></ path > < path d = "M7.5 13.5 10 11" ></ path > < path d = "M10.5 16.5 13 14" ></ path > < path d = "m18 3-4 4h6l-4 4" ></ path > < / > } } pub const LUCIDE_PLUG_ZAP : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;