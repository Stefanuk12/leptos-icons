use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 14a8 8 0 0 1-8 8" ></ path > < path d = "M18 11v-1a2 2 0 0 0-2-2a2 2 0 0 0-2 2" ></ path > < path d = "M14 10V9a2 2 0 0 0-2-2a2 2 0 0 0-2 2v1" ></ path > < path d = "M10 9.5V4a2 2 0 0 0-2-2a2 2 0 0 0-2 2v10" ></ path > < path d = "M18 11a2 2 0 1 1 4 0v3a8 8 0 0 1-8 8h-2c-2.8 0-4.5-.86-5.99-2.34l-3.6-3.6a2 2 0 0 1 2.83-2.82L7 15" ></ path > < / > } } pub const LUCIDE_POINTER : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;