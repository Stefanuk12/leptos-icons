use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 22a10 10 0 1 1 10-10" ></ path > < path d = "M22 22 12 12" ></ path > < path d = "M22 16v6h-6" ></ path > < / > } } pub const LUCIDE_CIRCLE_ARROW_OUT_DOWN_RIGHT : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;