use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 2H3v16h5v4l4-4h5l4-4V2zm-10 9V7m5 4V7" ></ path > < / > } } pub const LUCIDE_TWITCH : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none")] } ;