use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m20.9 18.55-8-15.98a1 1 0 0 0-1.8 0l-8 15.98" ></ path > < ellipse rx = "9" cx = "12" cy = "19" ry = "3" ></ ellipse > < / > } } pub const LUCIDE_CONE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;