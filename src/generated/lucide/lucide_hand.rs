use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 11V6a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v0" ></ path > < path d = "M14 10V4a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v2" ></ path > < path d = "M10 10.5V6a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v8" ></ path > < path d = "M18 8a2 2 0 1 1 4 0v6a8 8 0 0 1-8 8h-2c-2.8 0-4.5-.86-5.99-2.34l-3.6-3.6a2 2 0 0 1 2.83-2.82L7 15" ></ path > < / > } } pub const LUCIDE_HAND : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-linecap" , "round")] } ;