use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 5V19A9 3 0 0 0 15 21.84" ></ path > < path d = "M21 5V8" ></ path > < path d = "M21 12L18 17H22L19 22" ></ path > < path d = "M3 12A9 3 0 0 0 14.59 14.87" ></ path > < / > } } pub const LucideDatabaseZap : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;