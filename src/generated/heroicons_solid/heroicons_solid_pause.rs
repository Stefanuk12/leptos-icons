use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path clip - rule = "evenodd" d = "M6.75 5.25a.75.75 0 0 1 .75-.75H9a.75.75 0 0 1 .75.75v13.5a.75.75 0 0 1-.75.75H7.5a.75.75 0 0 1-.75-.75V5.25Zm7.5 0A.75.75 0 0 1 15 4.5h1.5a.75.75 0 0 1 .75.75v13.5a.75.75 0 0 1-.75.75H15a.75.75 0 0 1-.75-.75V5.25Z" fill - rule = "evenodd" ></ path > < / > } } pub const HEROICONS_SOLID_PAUSE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "currentColor") , ("aria-hidden" , "true") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon")] } ;