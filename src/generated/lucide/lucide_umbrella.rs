use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 12a10.06 10.06 1 0 0-20 0Z" ></ path > < path d = "M12 12v8a2 2 0 0 0 4 0" ></ path > < path d = "M12 2v1" ></ path > < / > } } pub const LUCIDE_UMBRELLA : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;