use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10.5 5H19a2 2 0 0 1 2 2v8.5" ></ path > < path d = "M17 11h-.5" ></ path > < path d = "M19 19H5a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2" ></ path > < path d = "m2 2 20 20" ></ path > < path d = "M7 11h4" ></ path > < path d = "M7 15h2.5" ></ path > < / > } } pub const LUCIDE_CAPTIONS_OFF : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;