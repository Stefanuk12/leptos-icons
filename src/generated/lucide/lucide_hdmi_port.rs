use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 9a1 1 0 0 0-1-1H3a1 1 0 0 0-1 1v4a1 1 0 0 0 1 1h1l2 2h12l2-2h1a1 1 0 0 0 1-1Z" ></ path > < path d = "M7.5 12h9" ></ path > < / > } } pub const LUCIDE_HDMI_PORT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;