use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 2v6" ></ path > < path d = "M15 2v6" ></ path > < path d = "M12 17v5" ></ path > < path d = "M5 8h14" ></ path > < path d = "M6 11V8h12v3a6 6 0 1 1-12 0Z" ></ path > < / > } } pub const LUCIDE_PLUG_2 : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round")] } ;