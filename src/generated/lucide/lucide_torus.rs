use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < ellipse ry = "2" rx = "3" cx = "12" cy = "11" ></ ellipse > < ellipse rx = "10" cx = "12" cy = "12.5" ry = "8.5" ></ ellipse > < / > } } pub const LUCIDE_TORUS : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none")] } ;