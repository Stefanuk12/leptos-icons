use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 2v7.31" ></ path > < path d = "M14 9.3V1.99" ></ path > < path d = "M8.5 2h7" ></ path > < path d = "M14 9.3a6.5 6.5 0 1 1-4 0" ></ path > < path d = "M5.52 16h12.96" ></ path > < / > } } pub const LUCIDE_FLASK_ROUND : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2")] } ;