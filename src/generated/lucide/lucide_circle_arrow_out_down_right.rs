use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 22a10 10 0 1 1 10-10" ></ path > < path d = "M22 22 12 12" ></ path > < path d = "M22 16v6h-6" ></ path > < / > } } pub const LUCIDE_CIRCLE_ARROW_OUT_DOWN_RIGHT : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;