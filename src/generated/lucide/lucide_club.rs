use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17.28 9.05a5.5 5.5 0 1 0-10.56 0A5.5 5.5 0 1 0 12 17.66a5.5 5.5 0 1 0 5.28-8.6Z" ></ path > < path d = "M12 17.66L12 22" ></ path > < / > } } pub const LUCIDE_CLUB : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2")] } ;