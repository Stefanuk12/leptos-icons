use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2.5 17a24.12 24.12 0 0 1 0-10 2 2 0 0 1 1.4-1.4 49.56 49.56 0 0 1 16.2 0A2 2 0 0 1 21.5 7a24.12 24.12 0 0 1 0 10 2 2 0 0 1-1.4 1.4 49.55 49.55 0 0 1-16.2 0A2 2 0 0 1 2.5 17" ></ path > < path d = "m10 15 5-3-5-3z" ></ path > < / > } } pub const LUCIDE_YOUTUBE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;