use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < ellipse cy = "5" rx = "9" cx = "12" ry = "3" ></ ellipse > < path d = "M3 5V19A9 3 0 0 0 15 21.84" ></ path > < path d = "M21 5V8" ></ path > < path d = "M21 12L18 17H22L19 22" ></ path > < path d = "M3 12A9 3 0 0 0 14.59 14.87" ></ path > < / > } } pub const LUCIDE_DATABASE_ZAP : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;