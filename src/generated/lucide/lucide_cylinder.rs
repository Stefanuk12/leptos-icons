use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < ellipse cy = "5" cx = "12" ry = "3" rx = "9" ></ ellipse > < path d = "M3 5v14a9 3 0 0 0 18 0V5" ></ path > < / > } } pub const LUCIDE_CYLINDER : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;