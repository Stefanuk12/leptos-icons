use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 20V8c0-2.2 1.8-4 4-4 1.5 0 2.8.8 3.5 2" ></ path > < path d = "M6 12h4" ></ path > < path d = "M14 12h2v8" ></ path > < path d = "M6 20h4" ></ path > < path d = "M14 20h4" ></ path > < / > } } pub const LUCIDE_LIGATURE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;