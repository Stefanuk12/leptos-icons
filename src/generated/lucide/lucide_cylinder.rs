use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < ellipse cy = "5" cx = "12" rx = "9" ry = "3" ></ ellipse > < path d = "M3 5v14a9 3 0 0 0 18 0V5" ></ path > < / > } } pub const LUCIDE_CYLINDER : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;