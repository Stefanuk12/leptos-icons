use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < ellipse rx = "9" cx = "12" ry = "3" cy = "5" ></ ellipse > < path d = "M3 5V19A9 3 0 0 0 21 19V5" ></ path > < path d = "M3 12A9 3 0 0 0 21 12" ></ path > < / > } } pub const LUCIDE_DATABASE : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;