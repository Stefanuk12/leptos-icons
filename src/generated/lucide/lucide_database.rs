use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < ellipse cx = "12" ry = "3" rx = "9" cy = "5" ></ ellipse > < path d = "M3 5V19A9 3 0 0 0 21 19V5" ></ path > < path d = "M3 12A9 3 0 0 0 21 12" ></ path > < / > } } pub const LUCIDE_DATABASE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;