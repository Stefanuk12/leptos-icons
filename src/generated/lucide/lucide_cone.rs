use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m20.9 18.55-8-15.98a1 1 0 0 0-1.8 0l-8 15.98" ></ path > < ellipse rx = "9" ry = "3" cx = "12" cy = "19" ></ ellipse > < / > } } pub const LUCIDE_CONE : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;