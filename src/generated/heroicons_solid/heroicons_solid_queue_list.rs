use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5.625 3.75a2.625 2.625 0 1 0 0 5.25h12.75a2.625 2.625 0 0 0 0-5.25H5.625ZM3.75 11.25a.75.75 0 0 0 0 1.5h16.5a.75.75 0 0 0 0-1.5H3.75ZM3 15.75a.75.75 0 0 1 .75-.75h16.5a.75.75 0 0 1 0 1.5H3.75a.75.75 0 0 1-.75-.75ZM3.75 18.75a.75.75 0 0 0 0 1.5h16.5a.75.75 0 0 0 0-1.5H3.75Z" ></ path > < / > } } pub const HEROICONS_SOLID_QUEUE_LIST : Path = Path { path : icon_path , props : & [("fill" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("aria-hidden" , "true") , ("data-slot" , "icon")] } ;