use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 18H2a4 4 0 0 0 4 4h12a4 4 0 0 0 4-4Z" ></ path > < path d = "M21 14 10 2 3 14h18Z" ></ path > < path d = "M10 2v16" ></ path > < / > } } pub const LUCIDE_SAILBOAT : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2")] } ;